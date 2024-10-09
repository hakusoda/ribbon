# 🎀 Ribbon
Ribbon is an experimental Roblox-centric Discord Bot developed by [HAKUMI](https://github.com/hakusoda), we're in the early stages of development!

[add to server](https://discord.com/oauth2/authorize?client_id=1255829220614209636)—keep up-to-date with development in our [Discord Server](https://discord.com/invite/rs3r4dQu9P)!

# Project Structure
The project consists of [several different crates](https://doc.rust-lang.org/cargo/reference/workspaces.html), some responsible for different components—or just utility.

| Crate                                                                 | Description                                                                                                                                                                              |
| --------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`ribbon`](/crates/ribbon/)                                           | The core program of Ribbon that initialises everything.                                                                                                                                  |
| [`ribbon_cache`](/crates/ribbon_cache/)                               | A basic cache responsible for storing & fetching data (ex. Discord Servers).                                                                                                             |
| [`ribbon_commands`](/crates/ribbon_commands/)                         | Defines all of Ribbon's commands.                                                                                                                                                  |
| [`ribbon_commands_core`](/crates/ribbon_commands_core/)               | The core of Ribbon's command system.                                                                                                                                                     |
| [`ribbon_commands_core_macros`](/crates/ribbon_commands_core_macros/) | Macros for [`ribbon_commands_core`](/crates/ribbon_commands_core/).                                                                                                                      |
| [`ribbon_emojis`](/crates/ribbon_emojis/)                             | Provides a convenient all-in-one Emoji struct, which is auto-generated from files in [`assets/emojis`](/assets/emojis/). Additionally, all emojis are automatically uploaded to Discord. |
| [`ribbon_emojis_macros`](/crates/ribbon_emojis_macros/)               | Macros for [`ribbon_emojis`](/crates/ribbon_emojis/).                                                                                                                                    |
| [`ribbon_frontend`](/crates/ribbon_frontend/)                         | A RESTful API utilising [`actix-web`](https://github.com/actix/actix-web), built for Ribbon's [website](https://github.com/ribbonette/website), and developers looking to integrate.     |
| [`ribbon_models`](/crates/ribbon_models/)                             | Defines most of the data structures used by Ribbon.                                                                                                                                      |
| [`ribbon_syncing`](/crates/ribbon_syncing/)                           | Provides the syncing functionality.                                                                                                                                                      |
| [`ribbon_util`](/crates/ribbon_util/)                                 | Contains global-constants and HTTP-fetching methods.                                                                                                                                     |

# Contributing
we're working on a guide, stay tuned!

# License
Ribbon is licensed under [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.en.html), see [`LICENSE`](/LICENSE) for more information.