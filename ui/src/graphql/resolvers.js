import { createZomeCall } from "../connection/holochainClient";

/*
Holochain structure:
- instance name: test-instance
- zome name: profile
- available zome functions:
-- list_public_profiles
-- search_username
-- create_public_profile
-- create_private_profile
ZomeCall Structure:
-- createZomeCall('<instance_name>/<zome_name>/<function_name>)(arguments)
* argument key should be the same as the declared input name in the holochain function declarations
*/

const resolvers = {
  Query: {
    isEmailRegistered: async (_, { email }) =>
      await createZomeCall("/test-instance/profile/is_email_registered")({
        email,
      }),
    isUsernameRegistered: async (_, { username }) =>
      await createZomeCall("/test-instance/profile/is_username_registered")({
        username,
      }),
  },

  Mutation: {
    createPrivateProfile: async (_, profileInput) =>
      await createZomeCall("/test-instance/profile/create_private_profile")({
        input: profileInput.profile_input,
      }),
    createPublicProfile: async (_, username) =>
      await createZomeCall("/test-instance/profile/create_public_profile")({
        input: username.profile_input,
      }),
    deleteProfile: async (_, username) =>
      await createZomeCall("/profiles/profile/delete_profile")({
        input: username.username
      })
  },
};

export default resolvers;
