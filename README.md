# climp
CLI Music Player

## Usage

Now you can save the audio file to a playlist`(~/.config/climp)` and play it from there.
Play the audio file with the flag `-s` to copy it to the playlist.

```
climp (-c or -p) <file>
```

### With no -d
```
Title: "Mitt Tempel Av Stierner Og Brennende Maaner"|(7.6splat mins)
Artist: "Djevel"
Album: "Naa Skrider Natten Sort"
Year: 2022
```

### With -d
```
 Title -- "Mitt Tempel Av Stierner Og Brennende Maaner" -- 7.6splat mins
Artist -- "Djevel"
 Album -- "Naa Skrider Natten Sort"
  Year -- 2022
```

```
Usage: climp [OPTIONS] --track <TRACK>

Options:
  -t, --track <TRACK>  Choose audio file
  -c, --current        Choose audio from current directory
  -p, --playlist       Choose track from playlist
  -d, --design         1/2 design
  -s, --save           Save track to playlist
  -h, --help           Print help
```
