/******************************************************************************
  This file is part of snailcrypt-cli.

  Copyright 2023 Richard Bäck <richard.baeck@snailcrypt.com>

  Permission is hereby granted, free of charge, to any person obtaining a copy
  of this software and associated documentation files (the "Software"), to deal
  in the Software without restriction, including without limitation the rights
  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  copies of the Software, and to permit persons to whom the Software is
  furnished to do so, subject to the following conditions:

  The above copyright notice and this permission notice shall be included in all
  copies or substantial portions of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
  SOFTWARE.
*******************************************************************************/

use assert_cmd::Command;

#[test]
fn encrypt_small_str() -> Result<(), Box<dyn std::error::Error>> {
	let plaintext: String = String::from("hello world");
	
	//=========================================================================
	// Perform encryption
    let mut cmd_encrypt = Command::cargo_bin("snailcrypt-cli")
		.unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		});

    let encrypted = cmd_encrypt
    						   .arg("-e")
							   .arg("2022-11-19T17:00:00+0100")			   
							   .arg("-f")
							   .write_stdin(plaintext.as_str())
							   .assert()
							   .success();
	let ciphertext: Vec<u8> = encrypted.get_output().stdout.to_owned();
	
	//=========================================================================
	// Perform decryption		   
	let mut cmd_decrypt = Command::cargo_bin("snailcrypt-cli")
		.unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		});
	
	
	cmd_decrypt.arg("-d")
			   .write_stdin(ciphertext.clone())
			   .assert()
			   .stdout(plaintext);
			   
	//=========================================================================
	// Perform hint extraction 
	let mut cmd_hint = Command::cargo_bin("snailcrypt-cli")
		.unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		});
	
	
	cmd_hint
		.arg("-dT")
		.write_stdin(ciphertext.clone())
		.assert()
		.stdout("");

    Ok(())
}

#[test]
fn encrypt_large_str() -> Result<(), Box<dyn std::error::Error>> {
	let plaintext: String = 
		String::from("Nullam eu ante vel est convallis dignissim.  Fusce suscipit, wisi nec facilisis facilisis, est dui fermentum leo, 
quis tempor ligula erat quis odio.  Nunc porta vulputate tellus.  
Nunc rutrum turpis sed pede.  Sed bibendum.  Aliquam posuere.  
Nunc aliquet, augue nec adipiscing interdum, lacus tellus malesuada 
massa, quis varius mi purus non odio.  Pellentesque condimentum, 
magna ut suscipit hendrerit, ipsum augue ornare nulla, non 
luctus diam neque sit amet urna.  Curabitur vulputate vestibulum 
lorem.  Fusce sagittis, libero non molestie mollis, magna orci 
ultrices dolor, at vulputate neque nulla lacinia eros.  Sed id ligula
quis est convallis tempor.  Curabitur lacinia pulvinar nibh.  Nam a sapien.");
	
	//=========================================================================
	// Perform encryption
    let mut cmd_encrypt = Command::cargo_bin("snailcrypt-cli")
		.unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		});

    let encrypted = cmd_encrypt
    						   .arg("-e")
							   .arg("2022-11-19T17:00:00+0100")			   
							   .arg("-f")
							   .write_stdin(plaintext.as_str())
							   .assert()
							   .success();
	let ciphertext: Vec<u8> = encrypted.get_output().stdout.to_owned();
	
	//=========================================================================
	// Perform decryption		   
	let mut cmd_decrypt = Command::cargo_bin("snailcrypt-cli")
		.unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		});
	
	
	cmd_decrypt.arg("-d")
			   .write_stdin(ciphertext.clone())
			   .assert()
			   .stdout(plaintext);
			   
	//=========================================================================
	// Perform hint extraction 
	let mut cmd_hint = Command::cargo_bin("snailcrypt-cli")
		.unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		});
	
	
	cmd_hint
		.arg("-dT")
		.write_stdin(ciphertext.clone())
		.assert()
		.stdout("");

    Ok(())
}

#[test]
fn encrypt_small_str_hint() -> Result<(), Box<dyn std::error::Error>> {
	let plaintext: String = String::from("hello world");
	let hint = String::from("This is a small hint.");
	
	//=========================================================================
	// Perform encryption
    let mut cmd_encrypt = Command::cargo_bin("snailcrypt-cli")
		.unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		});

    let encrypted = cmd_encrypt
    						   .arg("-e")
							   .arg("2022-11-19T17:00:00+0100")			   
							   .arg("-t")
							   .arg(hint.as_str())
							   .arg("-f")
							   .write_stdin(plaintext.as_str())
							   .assert()
							   .success();
	let ciphertext: Vec<u8> = encrypted.get_output().stdout.to_owned();
	
	//=========================================================================
	// Perform decryption		   
	let mut cmd_decrypt = Command::cargo_bin("snailcrypt-cli")
		.unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		});
	
	
	cmd_decrypt.arg("-d")
			   .write_stdin(ciphertext.clone())
			   .assert()
			   .stdout(plaintext);

	//=========================================================================
	// Perform hint extraction 
	let mut cmd_hint = Command::cargo_bin("snailcrypt-cli")
		.unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		});
	
	
	cmd_hint
		.arg("-dT")
		.write_stdin(ciphertext.clone())
		.assert()
		.stdout(hint);

    Ok(())
}

#[test]
fn encrypt_large_str_hint() -> Result<(), Box<dyn std::error::Error>> {
	let plaintext = 
		String::from("Nullam eu ante vel est convallis dignissim.  Fusce suscipit, wisi nec facilisis facilisis, est dui fermentum leo, 
quis tempor ligula erat quis odio.  Nunc porta vulputate tellus.  
Nunc rutrum turpis sed pede.  Sed bibendum.  Aliquam posuere.  
Nunc aliquet, augue nec adipiscing interdum, lacus tellus malesuada 
massa, quis varius mi purus non odio.  Pellentesque condimentum, 
magna ut suscipit hendrerit, ipsum augue ornare nulla, non 
luctus diam neque sit amet urna.  Curabitur vulputate vestibulum 
lorem.  Fusce sagittis, libero non molestie mollis, magna orci 
ultrices dolor, at vulputate neque nulla lacinia eros.  Sed id ligula
quis est convallis tempor.  Curabitur lacinia pulvinar nibh.  Nam a sapien.");
	let hint = 
		String::from("Nullam eu ante vel est convallis dignissim.  Fusce suscipit, wisi nec facilisis facilisis, est dui fermentum leo, 
quis tempor ligula erat quis odio.  Nunc porta vulputate tellus.  
Nunc rutrum turpis sed pede.  Sed bibendum.  Aliquam posuere.  
Nunc aliquet, augue nec adipiscing interdum, lacus tellus malesuada 
massa, quis varius mi purus non odio.  Pellentesque condimentum, 
magna ut suscipit hendrerit, ipsum augue ornare nulla, non 
luctus diam neque sit amet urna.  Curabitur vulputate vestibulum 
lorem.  Fusce sagittis, libero non molestie mollis, magna orci 
ultrices dolor, at vulputate neque nulla lacinia eros.  Sed id ligula
quis est convallis tempor.  Curabitur lacinia pulvinar nibh.  Nam a sapien.");
	
	//=========================================================================
	// Perform encryption
    let mut cmd_encrypt = Command::cargo_bin("snailcrypt-cli")
		.unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		});

    let encrypted = cmd_encrypt
    						   .arg("-e")
							   .arg("2022-11-19T17:00:00+0100")			   
							   .arg("-t")
							   .arg(hint.as_str())
							   .arg("-f")							   
							   .write_stdin(plaintext.as_str())
							   .assert()
							   .success();
	let ciphertext: Vec<u8> = encrypted.get_output().stdout.to_owned();
	
	//=========================================================================
	// Perform decryption		   
	let mut cmd_decrypt = Command::cargo_bin("snailcrypt-cli")
		.unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		});
	
	
	cmd_decrypt.arg("-d")
			   .write_stdin(ciphertext.clone())
			   .assert()
			   .stdout(plaintext);
			   
	//=========================================================================
	// Perform hint extraction 
	let mut cmd_hint = Command::cargo_bin("snailcrypt-cli")
		.unwrap_or_else(|error| {
			panic!("Error: {:?}", error);
		});
	
	
	cmd_hint
		.arg("-dT")
		.write_stdin(ciphertext.clone())
		.assert()
		.stdout(hint);

    Ok(())
}

#[test]
fn encrypt_fail_lockdate() -> Result<(), Box<dyn std::error::Error>> {
	let plaintext: String = String::from("hello world");
	
	//=========================================================================
	// Perform encryption
    let mut cmd_encrypt = Command::cargo_bin("snailcrypt-cli")
							.unwrap_or_else(|error| {
								panic!("Error: {:?}", error);
							});

    cmd_encrypt
    		.arg("-e")
			.arg("2022-11-19T17:00:00+0100")			   
			.write_stdin(plaintext.as_str())
			.assert()
			.failure()
			.stderr("Error: lock date \"2022-11-19T17:00:00+0100\" is in the past.\n");

    Ok(())
}
