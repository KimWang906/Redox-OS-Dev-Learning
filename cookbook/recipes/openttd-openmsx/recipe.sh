GIT=https://gitlab.redox-os.org/redox-os/openttd-openmsx.git

function recipe_version {
    echo "0.3.1"
    skip=1
}

function recipe_update {
    echo "skipping update"
    skip=1
}

function recipe_build {
    echo "skipping build"
    skip=1
}

function recipe_test {
    echo "skipping test"
    skip=1
}

function recipe_clean {
    echo "skipping clean"
    skip=1
}

function recipe_stage {
    mkdir -pv "$1/share/games/openttd/baseset/openmsx"
    cp -Rv ./* "$1/share/games/openttd/baseset/openmsx"
    skip=1
}

