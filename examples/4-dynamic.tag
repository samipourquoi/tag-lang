def $log($content) {
	/tellraw @a { "storage": "tag:runtime", "nbt": "vars[-1].content", "interpret": true }
}

$var := "hello world";
$log($var);
