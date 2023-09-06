# c'umwelten

This repository provides a framework to interface with haptic devices, specifically using the BOS1901 piezo actuator driver, via Python. It leverages a Raspberry Pi (or similar Linux-based device) as an intermediary for SPI communication.
- Stream waveforms to the BOS1901.
- Read back data from the device.
- Optimize and tweak device performance via internal registers.
- Create Python-based applications to control and test haptic feedback.

## Hardware Setup
- Haptic device(s) (Piezo actuators, LRAs, etc.)
- BOS1901 Piezo Actuator Driver.
- Raspberry Pi (or another Linux-based system with SPI support).

## Overview of files 

{device_driver.py} Interface with each device using its specific communication protocol   
{central.py} Coordinate between different haptic devices  
{pattern_lib.py} Define and store various haptic patterns  
{waveform_utils.py} Generate and visualize sine waves   
