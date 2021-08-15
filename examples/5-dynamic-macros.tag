def $log($content, interpret) {
	/tellraw @a { "storage": "tag:runtime", "nbt": "vars[-1].content", "interpret": #{interpret} }
}

$var := "hello world";
$log($var, true);
