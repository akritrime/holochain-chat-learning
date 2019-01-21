// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require("tape");
const { Config, Container } = require("@holochain/holochain-nodejs")

const dnaPath = "dist/bundle.json"

const aliceAgent = Config.agent("alice")
const bobAgent   = Config.agent("bob")
const dna        = Config.dna(dnaPath)

const aliceInstance = Config.instance(aliceAgent, dna);
const bobInstance   = Config.instance(bobAgent, dna)

const config    = Config.container([aliceInstance, bobInstance])
const container = new Container(config)

container.start()

const alice = container.makeCaller("alice", dnaPath)
const bob   = container.makeCaller("bob", dnaPath)

// await new Promise(r => setTimeout(r, 2000))
test('Creates new user for both bob and alice', (t) => {
    t.plan(2)
    const res1 = alice.call("users", "main", "create_user", { user: { handle: "Alice", email: "alice@test.net"}})
    const res2 = bob.call("users", "main", "create_user", {user: { handle: "Bob", email: "bob@test.net"}})
    // console.log(res1, res2)
    t.equal(res1.success, true);
    t.equal(res2.success, true)
    // t.equal(result, "expected result!")
  
    // ends this test
    t.end()
})
  
test('bob can create and send message to alice', async (t) => {
    t.plan(2)
    let user_address;
    console.log("Started")
    user_address = bob.call("users", "main", "get_current_user", {}).address;
    console.log("Got Bob")
    const message = bob.call("messages", "main", "create_message", { message: {content: "Hello, there."}, user_address})
  
    user_address = alice.call("users", "main", "get_current_user", {}).address;
    console.log("Message created")
    
    let res = {};
    
    while (!res.success) {
      res = alice.call("users", "main", "receive_message", {message_address: message.address});
      await new Promise(resolve => setTimeout(resolve, 1000))
      console.log(res)
    }

    t.equal(message.success, true);
    t.equal(res.success, true)
    // t.equal(result, "expected result!")
  
    // ends this test
    t.end()
})