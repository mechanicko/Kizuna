import { createZomeCall } from "../connection/holochainClient";

/*
Holochain structure:
- instance name: test-instance
- zome name: profiles
- available zome functions:
-- create_profile
-- get_all_agents
-- get_my_address
-- get_profile
ZomeCall Structure:
-- createZomeCall('<instance_name>/<zome_name>/<function_name>)(arguments)
* argument key should be the same as the declared input name in the holochain function declarations
*/

const resolvers = {
  Query: {
    allAgents: async () => {
      const all_agents = await createZomeCall(
        "/test-instance/profiles/get_all_agents"
      )();
      return all_agents.map((agent) => ({
        id: agent.agent_id,
        username: agent.username,
      }));
    },
    me: async () => {
      const agent_id = await createZomeCall(
        "/test-instance/profiles/get_my_address"
      )();
      const username = await createZomeCall(
        "/test-instance/profiles/get_username"
      )({
        agent_address: agent_id,
      });
      if (username) {
        return {
          id: agent_id,
          username,
        };
      } else {
        return {
          id: agent_id,
          username: null,
        };
      }
    },
  },

  Mutation: {
    createProfile: async (_, username) => {
      const username_entry = await createZomeCall(
        "/test-instance/profiles/set_username"
      )(username);
      return {
        id: username_entry.agent_id,
        username: username_entry.username,
      };
    },
    deleteProfile: async (_, username) =>
      await createZomeCall("/profiles/profile/delete_profile")({
        input: username.username,
      }),
    addContact: async (_, input) => {
      const contacts = await createZomeCall(
        "/test-instance/contacts/add_contact"
      )({username: input.username, timestamp: input.timestamp});
      return {
        agent_id: contacts.agent_id,
        timestamp: contacts.timestamp,
        contacts: contacts.contacts,
        blocked: contacts.blocked
      };
    },
    removeContact: async (_, input) => {
      const contacts = await createZomeCall(
        "/test-instance/contacts/remove_contact"
      )({username: input.username, timestamp: input.timestamp});
      return {
        agent_id: contacts.agent_id,
        timestamp: contacts.timestamp,
        contacts: contacts.contacts,
        blocked: contacts.blocked
      };
    },
    blockContact: async (_, input) => {
      const contacts = await createZomeCall(
        "/test-instance/contacts/block"
      )({username: input.username, timestamp: input.timestamp});
      return {
        agent_id: contacts.agent_id,
        timestamp: contacts.timestamp,
        contacts: contacts.contacts,
        blocked: contacts.blocked
      };
    },
    unblockContact: async (_, input) => {
      const contacts = await createZomeCall(
        "/test-instance/contacts/unblock"
      )({username: input.username, timestamp: input.timestamp});
      return {
        agent_id: contacts.agent_id,
        timestamp: contacts.timestamp,
        contacts: contacts.contacts,
        blocked: contacts.blocked
      };
    },
  },
};

export default resolvers;
