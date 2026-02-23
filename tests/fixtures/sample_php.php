<?php

/** @phpstan-ignore-next-line */
$x = someUntypedFunction();

$y = anotherFunction(); // @phpstan-ignore-line

// @phpstan-ignore argument.type
callWithWrongType($x);
