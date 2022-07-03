#!/usr/bin/env bash
cd $(dirname $BASH_SOURCE[0]);  # cd current directory

# BUGFIX: <link data-trunk rel="scss" href="src/counter.scss"/> fails to compile
# WORKAROUND: ./grass.sh is called as a watch/build [hook] in Trunk.toml

if [[ ! `command -v grass` ]]; then
  cargo install grass
fi

# Lazy compile with caching
# Find all .scss + .sass files, newer than .css (or if .css missing), then compile through grass
function grass_cached () {
  SCSS=$1
  CSS=$(echo $SCSS | sed 's/\.s[ac]ss$/.css/')
  if [ ! -f $CSS ] || [ "$SCSS" -nt "$CSS" ]; then
    echo "grass '$SCSS' > '$CSS'";
    grass "$SCSS" > "$CSS";
  fi;
}
export -f grass_cached
find . -name '*.s[ac]ss' | xargs -P0 -I{} bash -c 'grass_cached "$@"' _ {}


## BUG: GNU Parallel is not preinstalled on AWS - use xargs instead
#CSS='{=1 s/\.s[ac]ss$/.css/ =}'
#find . -name '*.s[ac]ss' |
#  parallel --group "if [ ! -f $CSS ] || [ {1} -nt $CSS ]; then echo 'grass {1} > $CSS'; grass {1} > $CSS; fi;"
