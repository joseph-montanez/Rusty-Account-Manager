import io::reader;
import io::reader_util;
import println = io::println;
import libc::*;
import str;

type tAccount = {
	name: str,
	email: str,
};

fn toCStr(cstr: *libc::c_char) -> *libc::c_char {
	ret cstr;
}

fn doWrite(accounts: [tAccount]) -> [tAccount] {
	//let filename: *libc::c_char;
	let filename: *c_char = str::as_c_str("accounts.dat", toCStr);
	let mode: *c_char = str::as_c_str("w", toCStr);
	io::println("I will now crash!");
	let file = fopen(filename, mode);
	io::println("pff as if you got this far...");
	fclose(file);
	io::println("closed");
	ret accounts;
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
	io::println("3) Save to File");
	io::println("4) Exit");

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
		doLogic(doWrite(accounts));
	}
	else if (line == "4") {
		io::println("Good Bye!");
	}
	
}

fn main(args: [str]) {
 	io::println("hello world from '" + args[0] + "'!");
 	let accounts: [tAccount] = [];

 	doLogic(accounts);
}