# Architecture

This document describes the architecture of this project.


## gui

The `gui` module contains classes to start a graphical user interface using the [skia](https://skia.org/) library to draw.
It redraws whenever changes to the text model happen or other events occur.


## lsp

This module is used to communicate with binaries using the `Language Server Protocol`.
More info can be found here: [microsoft.github.io/language-server-protocol](https://microsoft.github.io/language-server-protocol/)


## model

Contains the `Document` struct which holds the information about an open document.
It is immutable so every change creates a new `Document` instance.
We use immutable datastructures from the `im` module to make this memory and cpu efficient.
This allows for easy multithreading and .