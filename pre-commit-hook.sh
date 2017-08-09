BOLD_RED='\033[1;31m'
NORMAL='\033[0m'

if git rev-parse --verify HEAD >/dev/null 2>&1; then
    against=HEAD
else
    # empty tree
    against=4b825dc642cb6eb9a060e54bf8d69288fbee4904
fi

rust_files() {
    git diff --cached --name-only --diff-filter=AM -z $against | tr '\0' '\n' | grep "\.rs$"
}

RS_FILES=`rust_files`
if [[ -z `rust_files` ]]; then
    exit
fi

################################################################

exec 1>&2

cargo clippy
CLIPPY_EXIT_CODE=$?

if [[ $CLIPPY_EXIT_CODE != 0 ]]; then
    exit $CLIPPY_EXIT_CODE
fi

echo $RS_FILES | xargs -L 1 rustfmt --write-mode overwrite
echo $RS_FILES | xargs -L 1 git add

if [[ -z `rust_files` ]]; then
    printf "${BOLD_RED}error:${NORMAL} Commit is empty after formatting\n"
    exit 1
fi
