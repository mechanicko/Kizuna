import gql from "graphql-tag";

/* 
Schema
- Types
-- PublicProfile and PrivateProfile
-- PublicProfileEntry and PrivateProfileEntry
- Queries
-- searchResults()
-- listProfiles()
- Mutations
-- regsterUsername()
-- createProfile()
*/ 

export default gql`
  type PublicProfile {
    username: String
  }

  type PrivateProfile {
    first_name: String,
    last_name: String,
    email: String,
  }

  input PrivateProfileEntry {
    first_name: String,
    last_name: String,
    email: String
  }

  input PublicProfileEntry {
    username: String
  }

  type Query {
    searchUsername (profile_input: PublicProfileEntry): [PublicProfile],
    listProfiles: [PublicProfile]
  }

  type Mutation {
    registerUsername(profile_input: PublicProfileEntry): PublicProfile,
    createProfile(profile_input: PrivateProfileEntry): PrivateProfile
  }
`
