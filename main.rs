import println = io::println;
import file_writer = io::file_writer;
import io::*;
import str;

type tAccount = {
	name: str,
	email: str,
};

fn doWrite(accounts: ~[tAccount]) -> ~[tAccount] {
	alt file_writer("accounts.dat", ~[io::create]) {
		result::ok(file) {

		}
		_ { fail }
	};

	ret accounts;
}

fn doSearch(accounts: ~[tAccount]) -> ~[tAccount] {
	let account_size = vec::len(accounts);
	println(uint::str(account_size));
	vec::iter(accounts, |account| {
		if (str::contains(account.name, "Jo")) {
			println("Name: " + account.name);
		}
	}); 
	ret accounts;
}

fn doLogic(accounts: ~[tAccount]) {
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
		doLogic(accounts + ~[account]);
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

fn main(args: ~[str]) {
 	io::println("hello world from '" + args[0] + "'!");
 	let accounts: ~[tAccount] = ~[];

 	doLogic(accounts);
}