# Contribution guide

Thank you for considering contributing to to GRass.
There are a few things to consider before contributing.

First of all, be nice.
This should not be explained, but here are a some of things that I consider not nice:

- Personal attacks.
- Keep arguing after your suggestion has been rejected.
- Homophobia, transphobia, racism, or excluding any other group.

If you focus on the code exclusively you'll be fine.
I only block people as a last resort, so don't worry too much about it.

New programmers are especially welcome.
One of my primary motivators is to allow people to become the best version of themselves.
Which means I don't care how good at programming or FOSS contribution someone is.
For more information look at the section "How to contribute".
It explains everything from the very basics, so anyone can contribute.
And feel free to ask questions if you're still stuck.

## The merge process

My maintainer strategy is not ideal if you want consistency.
I program based on my motivation, which I don't directly control.
I may be working on a project all week.
Or ignore it for a month.

I do try to keep track of pull request.
I will try to reject or accept pull requests within some reasonable time.
However, I may not merge them immediately.
Once a PR has been accepted, you are free to stack another PR on top of it.
Stacking a PR means branching off from the PR to create a new PR.

I acknowledge that this strategy is not ideal.
This is why I only maintain projects which are still very early in development,
or otherwise don't require active development.
If the project get's more activity, then I will actively look for a better maintainer.

It's fine to open a PR without an issue if you think it makes sense.
But if you want to contribute a major feature you should probably communicate that early on.
To make sure you're not wasting your time.

And lastly, while not a requirement, I do prefer commits to be signed by a GPG key.
For more information on how to do this, read <https://git-scm.com/book/en/v2/Git-Tools-Signing-Your-Work>.

## How to contribute

The first place to start is by understanding how rust and cargo work.
There are 2 books for this:

- <https://doc.rust-lang.org/book/>
- <https://doc.rust-lang.org/cargo/>.

These contain all the information you need.
Alternatively, find a tutorial or something.

Next is some project specific structure.
Right now I don't use unsafe at all.
I do not believe that unsafe is required for this tool.
If you think it is, you should first show a hypothetical.
If you can show meaningful performance gains by using unsafe,
then I'll consider it.
However, due to inexperience I'd likely need some extra time to verify the correctness of any unsafe code.

Keep the code as simple and loosely coupled as possible.
Pass functions to modify behavior.
You may create 10 versions of the same data, if the requirements of each is slightly different.

### Tools

Try to remember to run `cargo fmt`, `cargo clippy`, and `cargo test` before submitting a pull request.
While I won't reject them right away, it does make for a cleaner git history.

### Systems

While I generally avoid complexity, some use cases can be justified.
One of these is wrapping the entire API in a custom strategy system.
This works as such:

1. Strategies are defined as traits
2. The strategy is implemented, possibly in various ways
3. The strategies are collected in a central strategy,
   called and ApiStrategy by convention.
4. The ApiStrategy implements Support traits, which expose the strategies.
5. The ApiStrategy is wrapped inside the Api struct
6. Each API function requires the Api struct with one or more implemented strategies.

Typically you have an ApiStrategy which implements all strategies.
However it's not strictly required.
This system serves a few purposes:

- This is used for testing, through a collection of mocking strategies.
- After version 1, additional features may be implemented using another strategy.
- The library may be extended by a third party crate using custom strategies.
