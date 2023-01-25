<div align=center>
<br/>
<img src="https://images.unsplash.com/photo-1568526381923-caf3fd520382?ixlib=rb-1.2.1&q=85&fm=jpg&crop=entropy&cs=srgb&w=3600" width="400">
</div>

# BeeConnect

The Eik team is excited to introduce BeeConnect – a student-driven project with the goal of creating a sensing-system for monitoring the beehives. The system will consist of a network of sensors that will collect a mesh of temperature data. This data will be transmitted to a database where it can be analyzed to hopefully improve bee health and honey production. 

The project is currently in the development phase, and we are working hard to create a prototype that can be tested in the field. We are confident that BeeConnect will be a valuable tool for beekeepers and will help contribute to the preservation of this important pollinator.


## Developing

### Conventional Commits

We can use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for commit messages, so that we can automatically generate changelogs.

Example commit message in the scope of backend code development:

```sh
git commit -m "feat(be): add new endpoint for getting all users"
```

Where `feat` is the type of commit, `be` is the scope of the commit, the scopes we can use are:
* `be` for backend code
* `mcu` for microcontroller code
* `docs` for documentation
* `fe` for frontend code (Coming...)

...

The types of commits we can use are:
* `feat` for new features
* `fix` for bug fixes

Read more about the types of commits [here](https://www.conventionalcommits.org/en/v1.0.0/#summary).
