# Contributing

## Branching

### Hub branches

We have two permanent "hub" branches:

`main` is where the stable releases live. This is the most exclusive branch and will only include changes that have gone
through rigorous testing.

`dev` is where completed features are compiled to.
Commits to `dev` are eventually merged into `main` as releases as
described
in [here](#making-releases).

### Work branches

All work is done in a work branch, not directly commited to either `main` or `dev`. They are named with a prefix to
describe their function. Here are all the prefixes that are used in this project with some examples:

- `feature/` is for new features
    - `feature/item-store`
    - `feature/persistence`
- `refactor/` is for code changes that do not add functionality
    - `refactor/api-endpoint`
    - `refactor/store-system`
- `bugfix/` is for bugfixes (duh)
    - `bugfix/nuclear-bomb-healing`
- `hotfix/` is for fixing bugs that are in `main` that can be merged back to `main` without having to wait for the next
  release
    - `hotfix/infinite-money`
- `docs/` is for changes that purely involve the documentation
    - `docs/make-license-evil`

After work on one of the work branches is complete, a pull request must be created to merge the changes from that branch
to the appropriate hub branch (usually `dev`) upon approval.

Of course, you are free to make as many branches as you like. Sub-branch your own branch or someone else's. Merge them
back into a single branch. All those are fine, as long as you are not merging changes directly into a hub branch without
a
pull request. However, **please be polite**. Although technically you can modify someone else's branch and even merge
your own
branch into it, please don't without their permission as you might mess up their workflow.

### Making releases

When we are ready to make a release, we make a branch off `dev` named with the release version (e.g., `release/v0.69`).
Contributors are still free to make pull requests to `dev` but must be aware that their changes will not be included in
the upcoming release.

We then test this release branch until we are sure that it's bug-free and upholds our standards. Hotfixes can be made
for this release via a `hotfix/` branch, and can be merged back into the release branch via a pull request.

After everything is ready and tested, it is merged into `main` and a **release tag** is made for it via GitHub's
releases feature (not just a basic git tag) and with a comprehensive changelog. All changes from this branch must also
be merged into `dev`. 
