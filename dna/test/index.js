/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

//  TATS: Watch 9:30 to 20:00 of this video from devcamp to know more about tryorama
//　https://www.youtube.com/watch?v=OX9jsY24S9A&list=PLJgZAXKruDXf9QbFzebpPvA0UV7e35OWQ&index=7&t=0s

const path = require('path')
const { Orchestrator, Config} = require('@holochain/tryorama')

const dnaPath = path.join(__dirname, "../dist/dna.dna.json")

const dna = Config.dna(dnaPath, 'kizuna_dna')
const conductorConfig = Config.gen(
  {
    kizuna_dna: dna
  },
  {
    // use a sim2h network (see conductor config options for all valid network types)
    // TATS: So sim2h is the simulated network that we use for testing our zome calls. 
    // More info on the video link I shared above. Basically acts like a switchboard that allows
    // 2 agents to talk to one another.
    // IMPORTANT: Make sure to open a new terminal and get inside the nix-shell and run 'sim2h-server'
    // to instantiate a sim2h_server
    network: {
      type: 'sim2h',
      sim2h_url: 'ws://localhost:8888',
    },
  })

// Instatiate a test orchestrator.
// It comes loaded with a lot default behavior which can be overridden, including:
// * custom conductor spawning
// * custom test result reporting
// * scenario middleware, including integration with other test harnesses
const orchestrator = new Orchestrator()

// Register a scenario, which is a function that gets a special API injected in
// TATS: this first line is just a boiler plate then sa string you can just specify what scenario you are creating 
orchestrator.registerScenario("call create_profile then get_profile", async (s, t) => {

  // Declare two players using the previously specified config,
  // and nickname them "alice" and "bob"
  // TATS: yung true na second argument is just saying the conductor for each of these 
  // test agents should be auto-spawned. If no arguemnt true is passed in the second arguemnt, then
  // you have to call await alice.spawn() to spawn the conductor yourself and kill it after the scenario 
  // with await alice.kill()
  const {alice, bob} = await s.players({alice: conductorConfig, bob: conductorConfig}, true)

  // Make a call to a Zome function create_profile
  // indicating the function, and passing it the argument
  // FROM TATS: the first argument is the nickname I assigned in line 13. then zome name then zome call name.
  const private_profile_addr = await alice.call("kizuna_dna", "profile", "create_private_profile", {
    first_name : "tatsuya",
    last_name: "sato",
    email: "tatsuya.g.sato@gmail.com"
  })

  // TATS: check if the profile_addr returns Ok from rust
  t.ok(private_profile_addr.Ok)

  // Wait for all network activity to settle
  await s.consistency()

  const public_profile_addr = await bob.call("kizuna_dna", "profile", "create_public_profile", {
    username : "tats_sato"
  })

  t.ok(public_profile_addr.Ok)

  await s.consistency()

  // TATS: now, let's try to get the entry content created with the result_create_profile with bob using get_profile call
  // the profile_addr.Ok contains the address of the profile entry committed since create_profile returns the addr of the committed entry. 
  const private_profile_result = await bob.call("kizuna_dna", "profile", "get_profile", {"address": private_profile_addr.Ok})

  // check for equality of the actual and expected results
  t.deepEqual(private_profile_result, { Ok: { App: [ 'PRIVATE_PROFILE', '{"first_name":"tatsuya","last_name":"sato","email":"tatsuya.g.sato@gmail.com"}' ] } })

  const public_profile_result = await alice.call("kizuna_dna", "profile", "get_profile", {"address": public_profile_addr.Ok})

  t.deepEqual(public_profile_result, { Ok: { App: [ 'PUBLIC_PROFILE', '{"username":"tats_sato"}' ] } })
})

orchestrator.run()
