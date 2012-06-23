import io::reader;
import io::reader_util;
import serialization;
import println = io::println;

type tAccount = {
	name: str,
	email: str,
};

type R = {x: uint, y: uint};
fn serialize2<C: serialization::ctxt>(cx: C, &&v: option<R>) {
    cx.emit_enum("std::option::t<R>") {||
        alt v {
            none {
                cx.emit_variant("std::option::none", 0u) {||
                }
            }
            some(r) {
                cx.emit_variant("std::option::some", 1u) {||
                    serialize1(cx, r); // link to the previous code we saw
                }
            }
        }
    }
}

fn doSearch(accounts: [tAccount]) -> [tAccount] {
	let account_size = vec::len(accounts);
	println(uint::str(account_size));
	vec::iter(accounts) {|account|
		if (str::contains(account.name, "Jo")) {
			println("Name: " + account.name);
		}
	}
	ret accounts;
}

fn doLogic(accounts: [tAccount]) {
	io::println("What would you like to do?");
	io::println("1) Add Account");
	io::println("2) Search For Account");
	io::println("3) Exit");

	let reader = io::stdin();
	let line = reader.read_line();

	if (line == "1") {
		io::print("Name: ");
		let name = reader.read_line();
		io::print("\nEmail: ");
		let email = reader.read_line();
		let account: tAccount = {
			name: name,
			email: email
		};
		io::println("Name: " + account.name + " Email: " + account.email);
		doLogic(accounts + [account]);
	}
	else if (line == "2") {
		doLogic(doSearch(accounts));
	}
	else if (line == "3") {
		io::println("Good Bye!");
	}
	
}

fn main(args: [str]) {
 	io::println("hello world from '" + args[0] + "'!");
 	let accounts: [tAccount] = [];

 	doLogic(accounts);
}