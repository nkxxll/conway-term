#!/usr/bin/env bash

rm ./test_games.sqlite

summary=""
TEST_DATABASE="test_games.sqlite" cargo run -- play -r 20 -s -b
TEST_DATABASE="test_games.sqlite" cargo run -- play -r 20 -s -b
TEST_DATABASE="test_games.sqlite" cargo run -- play -r 20 -s -b
TEST_DATABASE="test_games.sqlite" cargo run -- play -r 20 -s -b
TEST_DATABASE="test_games.sqlite" cargo run -- play -r 20 -s -b
TEST_DATABASE="test_games.sqlite" cargo run -- play -r 50
TEST_DATABASE="test_games.sqlite" cargo run -- database --list
get_one=$(TEST_DATABASE="test_games.sqlite" cargo run -- database --get 1)
get_two=$(TEST_DATABASE="test_games.sqlite" cargo run -- database --get 1)
if [ "$get_one" -ne "$get_two" ]
then
    echo "[0] test failed... one is not two"
    summary="$summary\n[0] test failed... one is not two"
    echo "$get_one"
    echo "$get_two"
else
    echo "[0] test passed"
    summary="$summary\n[0] test passed"

    echo "one is two"
fi
get_one=$(TEST_DATABASE="test_games.sqlite" cargo run -- database --get 2)
get_two=$(TEST_DATABASE="test_games.sqlite" cargo run -- database --get 2)
if [ "$get_one" -ne "$get_two" ]
then
    echo "[1] test failed... one is not two"
    summary="$summary\n[1] test failed... one is not two"
    echo "$get_one"
    echo "$get_two"
else
    echo "[1] test passed"
    summary="$summary\n[1] test passed"
    echo "one is two"
fi
echo "summary"
echo -e "$summary"
echo "testing graceful shutdown"
sleep 4
TEST_DATABASE="test_games.sqlite" cargo run -- play
