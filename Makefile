################################################################################
# This file is part of snailcrypt-cli. For more information visit
# https://www snailcrypt.com
# Copyright (C) 2022-2023  Richard Bäck <richard.baeck@icloud.com>
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License along
# with this program; if not, write to the Free Software Foundation, Inc.,
# 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
################################################################################

SYSTEM_NAME=$(shell uname -o)
ARCH_NAME=$(shell uname -m)

PROJECT_NAME=$(shell cargo pkgid | cut -d\# -f1 | xargs basename)
VERSION_NAME=$(shell cargo pkgid | cut -d\# -f2)
DIST_NAME=snailcrypt-cli_$(VERSION_NAME)_$(SYSTEM_NAME)_$(ARCH_NAME)

all:
	cargo auditable build --release

debug:
	cargo build
	
release:
	cargo build --release
	
check:
	cargo test

audit:
	cargo audit bin target/release/snailcrypt-cli
	
dist: release
	rm -f $(DIST_NAME).zip
	zip --junk-paths $(DIST_NAME).zip LICENSE README.md target/release/snailcrypt-cli
