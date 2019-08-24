for example in */ ; do
    echo "Building - ${example::-1}..."
    cd "$example"
    . clean.sh
    . build.sh
    cd ..
done