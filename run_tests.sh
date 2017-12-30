EXECUTABLE=${TEST_EXECUTABLE:-"./target/release/l3"}
TESTDIR=`mktemp -d` || (echo "can't make tempdir"; exit 1)
VERBOSE=${TEST_VERBOSE:-""}

FAILED=""

function test_pass {
	echo "$TESTNAME - $(tput setaf 2)pass$(tput sgr0)"
}

function test_fail {
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

if [ -n "$FAILED" ]
then
	echo some tests failed!
	exit 1
fi

exit 0
