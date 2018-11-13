# Rust Joueur Client

This is the client for the [Cadre][cadre] AI framework. It can play multiple different games, though you will probably only be interested in one at a time.

In general, try to stay out of the `joueur/` folder, it does most of the heavy lifting to play on our game servers.

Each AI, and the game objects it manipulates are all in `games/game_name/`, with your very own AI living in `games/game_name/ai.hpp` and `games/game_name/ai.cpp` files for you to make smarter.

## How to Run

This client has been tested and confirmed to work on the Campus rc##xcs213 Linux machines, but it can work on your own Windows/Linux/Mac machines if you desire.

Also make sure **NOT** to try to compile this in your Missouri S&T S-Drive. This is not a fault with the client, but rather the school's S-Drive implementation changing some file permissions during run time. We cannot control this. Instead, we recommend cloning your repo outside the S-Drive and use an SCP program like [WinSCP][winscp] to edit the files in Windows using whatever IDE you want if you want to code in Windows, but compile in Linux.

### Linux

    make
    ./testRun MyOwnGameSession

Linux does not have any dependencies beyond a C++ compiler and build system. You will need `make` and `cmake` to build, and `gcc` for compiling.

### Windows

## Other notes

Most importantly, **stay out of the impl/ directories**.

[vs]: https://www.visualstudio.com/downloads/
[cmake]: http://cmake.org/
[mingw]: http://www.mingw.org/
[winscp]: https://winscp.net/eng/download.php
[vagrant]: https://www.vagrantup.com/downloads.html
[virtualbox]: https://www.virtualbox.org/wiki/Downloads
[vagrant-guide]: https://www.vagrantup.com/docs/getting-started/up.html
[virtualbox]: https://www.virtualbox.org/wiki/Downloads
[gitbash]: https://git-scm.com/downloads
