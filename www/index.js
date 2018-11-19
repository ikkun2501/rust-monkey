import {Repl} from "rust-monkey";
import * as $ from 'jquery';

let repl = Repl.new();
let output = repl.input("let aiueo");
console.log(output);
$('textarea').text(output);
