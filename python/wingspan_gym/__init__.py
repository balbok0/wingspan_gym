"""Wingspan Environment is meant to simulate board game Wingspan in compute-friendly way."""

from .game import WingspanEnv

from ._internal import StepResult

__all__ = ["StepResult", "WingspanEnv"]
