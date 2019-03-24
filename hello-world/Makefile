
.SUFFIXES:

ifeq ($(strip $(DEVKITPRO)),)
$(error "Please set DEVKITPRO in your environment. export DEVKITPRO=<path to>/devkitpro")
endif

TOPDIR ?= $(CURDIR)
ROOTDIR ?= $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

include $(DEVKITPRO)/libnx/switch_rules

# Modded devkitPro's Makefile for Rust/cargo/xargo support!

# --------------------------------------------------------------------------------------

# The rest of customizable assets (name, authors, version...) is on the TOML file(s).

# Toolchain, should stay as aarch64-none-elf
TOOLCHAIN := aarch64-none-elf

# Mode: debug/release
MODE := release

# --------------------------------------------------------------------------------------

BUILD := target
OUTPUT := $(CURDIR)/$(BUILD)/$(TOOLCHAIN)/$(MODE)/

ifneq ($(BUILD),$(notdir $(CURDIR)))

ifeq ($(strip $(ICON)),)
	icons := $(wildcard *.jpg)
	ifneq (,$(findstring $(TARGET).jpg,$(icons)))
		export APP_ICON := $(TOPDIR)/$(TARGET).jpg
	else
		ifneq (,$(findstring icon.jpg,$(icons)))
			export APP_ICON := $(TOPDIR)/icon.jpg
		endif
	endif
else
	export APP_ICON := $(TOPDIR)/$(ICON)
endif

ifeq ($(strip $(NO_ICON)),)
	export NROFLAGS += --icon=$(APP_ICON)
endif

ifneq ($(APP_TITLEID),)
	export NACPFLAGS += --titleid=$(APP_TITLEID)
endif

ifneq ($(ROMFS),)
	export NROFLAGS += --romfsdir=$(CURDIR)/$(ROMFS)
endif

.PHONY: $(BUILD) clean all

#---------------------------------------------------------------------------------
all: $(BUILD)

$(BUILD):
	@[ -d $@ ] || mkdir -p $@
	@$(MAKE) --no-print-directory -C $(BUILD) -f $(CURDIR)/Makefile

#---------------------------------------------------------------------------------
clean:
	@echo clean ...
	@cargo clean
	@xargo clean
	@rm -fr $(OUTPUT).pfs0 $(OUTPUT).nso $(OUTPUT).nro $(OUTPUT).nacp $(OUTPUT).elf


#---------------------------------------------------------------------------------
else

TOPDIR := $(CURDIR)/..

XARGO_OUT := $(shell CARGO_INCREMENTAL=0 RUST_TARGET_PATH=$(TOPDIR) RUST_BACKTRACE=1 xargo build --$(BUILD) $(TOOLCHAIN) --$(MODE))
APP_TITLE := $(shell cat $(TOPDIR)/$(BUILD)/pkg.name)
APP_VERSION := $(shell cat $(TOPDIR)/$(BUILD)/pkg.version)
APP_AUTHOR := $(shell cat $(TOPDIR)/$(BUILD)/pkg.authors)
OUTPUT :=$(TOPDIR)/$(BUILD)/$(TOOLCHAIN)/$(MODE)/$(APP_TITLE)

ifeq ($(strip $(NO_NACP)),)
	export NROFLAGS += --nacp=$(OUTPUT).nacp
endif

.PHONY: all

#---------------------------------------------------------------------------------
# main targets
#---------------------------------------------------------------------------------

all	:	xargo $(OUTPUT).nso $(OUTPUT).nro

xargo:
	@echo compiled and linked ... $(OUTPUT).elf

$(OUTPUT).pfs0	:	$(OUTPUT).nso

$(OUTPUT).nso	:	$(OUTPUT).elf

ifeq ($(strip $(NO_NACP)),)
$(OUTPUT).nro	:	$(OUTPUT).elf $(OUTPUT).nacp
else
$(OUTPUT).nro	:	$(OUTPUT).elf
endif

endif