v0.1.0 (PRE-RELEASE) (WORK-IN-PROGRESS)
- esc deselects all
- ctrl click to select multiple
- global consistent usage of the term channel, which meets the official DMX spec
- made a dmxchannel type which is a niche-optimized type-safe represenation of a human-indexed channel number
- click again to deselect
- swaped from a vec to a [u64; 8] bitmask of selected channels
- neated up ordering of message
- shift for group select mode

TODO
- make scrolling when hovered over a channel increase it
- add an icon
- output over art-net
- toggle between 0-100 and 0-255 mode
- check if ctrl keycode works as cmd on mac
- alt for fine control with scroll
- consider multiuniverse setups
- help/shortcuts menu
- art-net discovery
- f11 to toggle fullscreen
- appear as an artnet endpoint