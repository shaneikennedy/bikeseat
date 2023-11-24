# Bikeseat

A learning exercise to see if I can replace my own blog [JuniorDeveloperDiaries](https://juniordeveloperdiaries.com) which uses netlify and gastby for simple static webhosting, with something that has the bare miniumum of what I need for a blog that has 30-60 monthly readers (aka not a lot).

## Goals
- Write it in Rust, I think this is a project that's not too hard but not too easy either
- Similar workflow, writing a new post means a new markdown file in some content directory. Home page, pagination should all work.
- Write the markdown parser from scratch, no libraries, and convert it to html that looks as bad as https://juniordeveloperdiaries does
- Actual CI/CD, not to the level of Netlify, but good enough. I never use the preview feature for Netlify anyways.
- Get real analytics, I don't want to pay $14/month or whatever it is for Netlify, and google analytics is particularly poorly suited for a community where 90% of the users probably have a tracking/ad-blocker.
