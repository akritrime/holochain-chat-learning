// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const alice = Container.instanceFromNameAndDna("alice", "dist/bundle.json");
const bob = Container.instanceFromNameAndDna("bob", "dist/bundle.json");

// activate the new instance
alice.start()
bob.start()

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
  
  user_address = bob.call("users", "main", "get_current_user", {}).address;
  const res1 = bob.call("messages", "main", "create_message", { message: {content: "Hello, there."}, user_address})

  user_address = alice.call("users", "main", "get_current_user", {}).address;
  const message_address = res1.address;

  
  let res2 = {};
  while (!res2.success) {
    res2 = bob.call("messages", "main", "send_message", {message_address, user_address});
    await new Promise(resolve => setTimeout(resolve, 1000))
    console.log(res2)
  }
  t.equal(res1.success, true);
  t.equal(res2.success, true)
  // t.equal(result, "expected result!")

  // ends this test
  t.end()
})
