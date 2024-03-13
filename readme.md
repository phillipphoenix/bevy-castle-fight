# Bevy Template

This template comes with an empty Bevy project along with Github Actions for CI and release.

On each push to the main branch or for each pull request created towards the main branch, the project is build and tested.

Creating a release in Github will start the release pipelines, which can create builds for Windows, Mac, Linux and WaSM (Web), which are uploaded to the Github release. It can also be set up to deploy directly to itch.io, if wanted.

## Notes about the pipelines

The CI and release actions are modified versions of the examples in the Bevy CI template repo:

https://github.com/bevyengine/bevy_github_ci_template

A few changes were necessary to make the actions run.

In the CI pipeline, Clippy needs to have added permissions to run:

`permissions: write-all`

This should be at the top level of the job.

Furthermore, each job that uploads a release in the release pipeline also needs added permissions to run:

`permissions: write-all`

This should be at the top level of the job.

This has been included in the template.
