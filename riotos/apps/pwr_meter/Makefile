# name of your application
APPLICATION = application_name

# If no BOARD is found in the environment, use this default:
BOARD ?= board-name
# BOARD ?=
# This has to be the absolute path to the RIOT base directory:
RIOTBASE ?= $(CURDIR)/../..

#USEMODULE += shell
#USEMODULE += shell_commands
#USEMODULE += ps
USEMODULE += od
USEMODULE += xtimer
#...
#...
USEMODULE += rtest

#...
#...

# vvvvvv  Add this string vvvvvvvv
ARCHIVES += $(CARGO_LIB)

include $(RIOTBASE)/Makefile.include
