# sali

> sali is a toolkit to easily build greetd frontends

The goal of this project is to create an easy-to-use and easy-to-understand way
of builing greetd frontends using gtk4 components. It allows to create a login
frontend in a fully declarative way without writing any code whilst it's main
features are:

- Creating a custom layout tree using predefined components
- Selecting between one or more run configurations

## Dependencies

You'll need the following dependencies installed on your system:

- `gtk4`
- `gtk4-layer-shell`
- `gio`

> Depending on your distribution the names may differ, the above names are for
> the Arch and AUR packages

## Configuration

Everything is configured through a yaml configuration file which is per default
located at `~/.config/sali/config.yaml`. A custom path can be specified using
the `--config` or `-c` argument. It supports the following fields:

| Field            | Description                                                                                     | Default value     |
| ---------------- | ----------------------------------------------------------------------------------------------- | ----------------- |
| `monitors`       | Map of monitor configurations which assigns a name for each [monitor](#monitor)                 | -                 |
| `main_monitor`   | Name of the monitor on which the layout tree should be rendered                                 | -                 |
| `username`       | Optional default username which is prefilled in the username field                              | _none_            |
| `styles`         | Array of paths from where to load stylesheets. It supports `css` as well as `scss` stylesheets  | `[]`              |
| `classes`        | Css class names which are used to indicate some states, the fields are defined [here](#classes) | _Default Classes_ |
| `runners`        | Map of run configurations which assigns a name for each [runner](#runner)                       | -                 |
| `default_runner` | Name of the runner which should be preselected in the runner dropdown                           | _none_            |
| `layout`         | The [layout tree](#layout) of the window on the main monitor                                    | _Default Layout_  |

The following is an example configuration:

```yaml
monitors:
  left:
    output: DP-1
    background: /path/to/left/background/image.jpg
  center:
    output: DP-2
    background: /path/to/center/background/image.jpg
  right:
    output: DP-3
    background: [255, 0, 255]

main_monitor: center

styles: [ "~/.config/sali/styles.scss" ]

runners:
  hyprland:
    display_name: Hyprland
    run: [ "exec Hyprland &> /tmp/hyprland-session.log" ]

default_runner: hyprland

layout:
  type: box
  halign: fill
  valign: fill
  children:
    - type: box
        classes: [ "container" ]
        children:
        - type: datetime
    - type: box
        classes: [ "container" ]
        children:
        - type: username
        - type: password
```

### Monitor

A monitor is a representation of a display output. If a `background` is set a
new background layer window is created for this monitor

| Field        | Description                                                                                                    | Default value |
| ------------ | -------------------------------------------------------------------------------------------------------------- | ------------- |
| `output`     | Name of the display output on this monitor (e.g. `DP-1`)                                                       | -             |
| `background` | Background of the monitor, either path to a background image or rgb color in array form (e.g. `[255, 0, 255]`) | -             |

### Classes

A set of css classes which are applied to windows which can't be set through the
layout tree or which are applied to components based on some conditions

| Field         | Description                                                                             | Default value    |
| ------------- | --------------------------------------------------------------------------------------- | ---------------- |
| `background`  | Css classes which are applied to the background windows                                 | `["background"]` |
| `window`      | Css classes which are applied to the window holding the layout tree                     | `["window"]`     |
| `field_error` | Css class which is applied to the username/password fields when there was a login error | `error`          |
| `field_empty` | Css class which is applied to the username/password fields when they are empty          | `empty`          |

### Runner

A run configuration which is executed when a login attempt succeeds

| Field          | Description                                                                                | Default value |
| -------------- | ------------------------------------------------------------------------------------------ | ------------- |
| `display_name` | The name displayed in the [runner dropdown](#runner-dropdown) when this runner is selected | -             |
| `run`          | A list of commands to run when a login attempt with this runner succeeds                   | -             |
| `env`          | A list of environment variables to set for the commands                                    | `[]`          |

## Layout

The layout is the main part of the configuration. It specifies a node tree in
which your UI components should be ordered. Each component directly maps to a
gtk4 component which should make styling pretty straight forward.

In the following section you'll see a detailed documentation of all components
available. Should a gtk4 component you need be missing feel free to open an
issue. Please keep in mind the goal of this project is **not to become the next
gtk wrapper for creating widgets** like [ags](https://github.com/Aylur/ags) or
[eww](https://github.com/elkowar/eww) but rather be a toolkit for easily
creating greetd frontends using gtk

### Box

The box component is the main building block for any layout. It is the only
component which can hold children and it can be used for alignment

A box component can be added to the layout tree using `type: box` and the
attributes below. Internally it's used to create an
[Box](https://docs.gtk.org/gtk4/class.Box.html) widget

| Attribute     | Description                                                                           | Default value |
| ------------- | ------------------------------------------------------------------------------------- | ------------- |
| `classes`     | Css classes which are applied to the box                                              | `["box"]`     |
| `halign`      | Horizontal alignment, either: `center`, `fill`, `start`, `end`, `baseline` or `start` | `center`      |
| `valign`      | Vertical alignment, either: `center`, `fill`, `start`, `end`, `baseline` or `start`   | `center`      |
| `hexpand`     | Boolean whether the box should expand horizontally                                    | `true`        |
| `vexpand`     | Boolean whether the box should expand vertically                                      | `true`        |
| `width`       | Requested width in pixels                                                             | `500`         |
| `spacing`     | Spacing between the children of the box in pixels                                     | `12`          |
| `orientation` | Orientation of the children, either: `vertical` or `horizontal`                       | `vertical`    |
| `children`    | Array of components which are nested inside the box                                   | `[]`          |

### Password field

The password field is a mandatory component which is used to enter the password
for the user. It can be added to the layout tree using `type: password` and the
attributes below. Internally it's used to create an
[Entry](https://docs.gtk.org/gtk4/class.Entry.html) widget which has it's
characters set to invisible

> [!NOTE]
>
> The password field is an unique component: If it's defined multiple times only
> the first password field in the layout tree is used and the other ones are
> ignored

| Attribute     | Description                                         | Default value           |
| ------------- | --------------------------------------------------- | ----------------------- |
| `classes`     | Css classes which are applied to the password field | `["input", "password"]` |
| `placeholder` | Placeholder text                                    | `Password`              |

### Username field

The username field is an optional component which is used to enter the username
for the user to log in. Should no `username` be specified in the config the
component becomes mandatory since it's not known for which user the login
attempt is otherwise. If an `username` is set in the config it's value is
prefilled into the input field

The username field can be added to the layout tree using `type: username` and
the attributes below. Internally it's used to create an
[Entry](https://docs.gtk.org/gtk4/class.Entry.html) widget

> [!NOTE]
>
> The username field is an unique component: If it's defined multiple times only
> the first username field in the layout tree is used and the other ones are
> ignored

| Attribute     | Description                                         | Default value           |
| ------------- | --------------------------------------------------- | ----------------------- |
| `classes`     | Css classes which are applied to the username field | `["input", "username"]` |
| `placeholder` | Placeholder text                                    | `Username`              |

### Runner dropdown

The runner dropdown is an optional component which can be used to select which
runner should be used on a successful login attempt. Should no `default_runner`
be specified in the config the component becomes mandatory since it's not known
which runner to use otherwise. If a `default_runner` is set in the config it's
value is preselected in the dropdown

The runner dropdown can be added to the layout tree using `type: runner` and the
attributes below. Internally it's used to create a
[DropDown](https://docs.gtk.org/gtk4/class.DropDown.html) widget

> [!NOTE]
>
> The runner dropdown is an unique component: If it's defined multiple times
> only the first runner dropdown in the layout tree is used and the other ones
> are ignored

| Attribute | Description                                          | Default value            |
| --------- | ---------------------------------------------------- | ------------------------ |
| `classes` | Css classes which are applied to the runner dropdown | `["runner", "dropdown"]` |

### DateTime label

The datetime label component can be used to display the current date or time. It
can be added to the layout tree using `type: datetime` and the attributes below.
Interally, it's used to create a
[Label](https://docs.gtk.org/gtk4/class.Label.html) widget

| Attribute  | Description                                                                                       | Default value           |
| ---------- | ------------------------------------------------------------------------------------------------- | ----------------------- |
| `classes`  | Css classes which are applied to the datetime label                                               | `["label", "datetime"]` |
| `format`   | Date format in [strftime syntax](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) | `%H:%M`                 |
| `interval` | Intervall in which the label should be updated in milliseconds                                    | `1000`                  |

### Label

The label component can be used to display some static text. It can be added to
the layout tree using `type: label` and the attributes below. Interally, it's
used to create a [Label](https://docs.gtk.org/gtk4/class.Label.html) widget

| Attribute | Description                                | Default value |
| --------- | ------------------------------------------ | ------------- |
| `classes` | Css classes which are applied to the label | `["label"]`   |
| `label`   | Text of the label                          | -             |
