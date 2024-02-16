# Contributing

Welcome to the GitLab bindings crate for Rust! This crate aims to offer
definitions of all of GitLab's API endpoints. The API patterns involved are
documented in [this blog post][designing-rust-bindings-for-rest-apis].

[designing-rust-bindings-for-rest-apis]: https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is

This design means that this crate aims to define:

- basic endpoint types
- endpoint combinators
- authentication mechanisms
- endpoint definitions themselves

It also means that it does *not* provide:

- definitions for response types
- server-version-aware variants of the API

Eventually, the API should adhere to semver, but it is hard to give a timeline.

## Type definitions and webhooks

Older releases of this crate tried to define endpoints as methods and define
the response types. This did not scale well. As such, the `types` module is
deprecated as of `0.1609.0`.

The `hooks`, `systemhooks`, and `webhooks` are hard to provide semver
guarantees and may be migrated to their own crate in the future.

None of the hook or types modules provide any semver guarantees.

## Guidelines

### Implementing an endpoint

- [ ] Add an entry to `CHANGELOG.md`. If there is no new section after a
      release, add a new section with a bumped patch number and ` (unreleased)`
      after the new version number.
- [ ] Update `src/api/README.md`:
  - [ ] Move the endpoint from the `Todo` section to the `Implemented` section,
        keeping things sorted.
  - [ ] If implementing an API from a page in the `Endpoint groups` section,
        please add all unimplemented endpoints to the `Todo` list and remove
        the entry from the list of files.
- [ ] All `Endpoint` structs must be `Clone`.
- [ ] Use `#[builder(setter(strip_option))]` if-and-only-if the endpoint has
      `Option` parameters.
- [ ] For `BTreeSet` and/or `Vec` parameters, make the default builder methods
      private and offer:
      - [ ] a method to reset and/or remove items
      - [ ] a method to add a single item
      - [ ] a method to add a sequence of items from an `Iterator` (if
            sensible)
- [ ] Test that each required parameter is actually required by providing all
      required parameters except the parameter under test.
- [ ] For each optional parameter, add a test which provides the required
      parameters and the single optional parameter.
- [ ] Expose the endpoint, its builder, and its builder error in the parent
      module.
- [ ] Consider whether the endpoint should `impl Pageable` or not.

### Adding a new parameter

- [ ] Add an entry to `CHANGELOG.md`. If there is no new section after a
      release, add a new section with a bumped patch number and ` (unreleased)`
      after the new version number.
- [ ] All `enum` types backing parameters should (generally) be
      `#[non_exhaustive]` because GitLab can add new variants at any time.
- [ ] Add tests for all `enum` types implementing `ParamQuery` and `Default`.
- [ ] Expose any new `enum` types in the parent module.
