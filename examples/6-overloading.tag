def $log($content, interpret) {
	/tellraw @a { "storage": "tag:runtime", "nbt": "vars[-1].content", "interpret": #{interpret} }
}

def $log($content) {
	$log($content, true);
}

$var := "hello world";
$log($var);
