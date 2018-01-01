#!/bin/bash
EXECUTABLE=${TEST_EXECUTABLE:-"./target/debug/l3"}
VERBOSE=${TEST_VERBOSE:-""}
TESTDIR=`mktemp -d` || (echo "can't make tempdir"; exit 1)

FAILED=""
NUM_PASS="0"
NUM_FAILED="0"

function test_pass {
	NUM_PASS=$((NUM_PASS + 1))
	echo "$TESTNAME - $(tput setaf 2)pass$(tput sgr0)"
}

function test_fail {
	NUM_FAILED=$((NUM_FAILED + 1))
	FAILED="${FAILED} ${TESTNAME}"
	echo "$TESTNAME - $(tput setaf 1)fail$(tput sgr0)"
	if [ -n "$VERBOSE" ]
	then
		echo return value: $RETVAL
		echo output: 
		cat $TESTDIR/out
		echo expected:
		cat tests/${TESTNAME}.out
		echo errors:
		cat $TESTDIR/err
	fi
}

function results {
	RESULTS="$((NUM_PASS)) / $((NUM_PASS + NUM_FAILED)) passed"
	if [ -n "$FAILED" ] 
	then
		echo $(tput setaf 1)FAILED$(tput sgr0) - $RESULTS
	else
		echo $(tput setaf 2)OK$(tput sgr0) - $RESULTS
	fi
}

function run {
	$EXECUTABLE $TEST > $TESTDIR/out 2> $TESTDIR/err 
}

function test_cleanup {
	rm $TESTDIR/*
}

for TEST in tests/*.l3
do
	BASENAME=`basename $TEST`
	TESTNAME=${BASENAME%.*}
	if run
	then
		RETVAL="$?"
		if cmp --silent tests/${TESTNAME}.out $TESTDIR/out
		then
			test_pass
		else
			test_fail
		fi
	else
		RETVAL="$?"
		test_fail
	fi
	test_cleanup
done

for TEST in tests/fail/*.l3
do
	BASENAME=`basename $TEST`
	TESTNAME=${BASENAME%.*}
	if run
	then
		RETVAL="$?"
		test_fail
	else
		RETVAL="$?"
		test_pass
	fi
	test_cleanup
done

rm -r $TESTDIR

results

if [ -n "$FAILED" ]
then
	exit 1
fi

exit 0
