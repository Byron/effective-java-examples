package org.effectivejava.examples.chapter04.Item20.hierarchy;

class Rectangle implements Figure {
	final double length;
	final double width;

	Rectangle(double length, double width) {
		this.length = length;
		this.width = width;
	}

	public double area() {
		return length * width;
	}
}
