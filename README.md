# File Browser

This is a small rust based file browser I am toying with in some of my spare time. It uses [hypertext](https://crates.io/crates/hypertext) as its templating engine and [htmx](https://htmx.org) for client side behavior.

In future this service may be expanded to multi user support with actual editing capabilities. But for now all it does is show you the file system and let you view the contents on the files on it.


## Future plans

- [ ] Add ability to edit files and save
- [ ] Add authentication and user management
- [ ] Handle non-text file types such as images, video & audio
- [ ] OAuth2 integration to allow for SSO
- [ ] Greatly improve UI/UX and create a nice framework for building UI's
- [ ] Switch from Monaco to [CodeMirror](https://codemirror.net/) given its mobile support
