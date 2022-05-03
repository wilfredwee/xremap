Gnome shell feature: https://gitlab.gnome.org/GNOME/gnome-shell/-/issues/new?issue%5Bmilestone_id%5D=



### Feature summary
I'd like to get the current active application in the foreground. I don't need to know the application's name, but the application identifier should be stable. (expanded more below)

AFAIK the previous methods of grabbing the `application-id` from either the `Eval` or `Introspect` method no longer works without an unsafe context: https://gitlab.gnome.org/GNOME/gnome-shell/-/issues/3943





### How would you like it to work
I'd like GNOME Shell (or maybe in the `xdg-desktop-portal`?) to provide a new DBus API which allows for 'Safe Introspection'.

The key idea is to provide as much information as possible about running applications, *without* compromising on user's privacy.


### Relevant links, screenshots, screencasts etc.

<!-- 
If you have further information, such as technical documentation,
code, mockups or a similar feature in another desktop environments,
please provide them here.
-->


<!-- Do not remove the following line. -->
/label ~"1. Feature"
