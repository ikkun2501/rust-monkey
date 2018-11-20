import {Repl} from "rust-monkey";
import * as $ from 'jquery';


let repl = Repl.new();

$(function () {
    $('#inputBtn').on('click', () => {
        let output = repl.input($('#input').val());
        $('#output').val($('#output').val() + output);
    });
});
