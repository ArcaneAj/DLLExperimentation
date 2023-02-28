// dllmain.cpp : Defines the entry point for the DLL application.
#include "pch.h"
#define EXPORT extern "C" __declspec(dllexport)

EXPORT int multiply(int x, int y) {
	return x * y;
}