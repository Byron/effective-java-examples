package org.effectivejava.examples.chapter04.Item20.hierarchy;

class Circle implements Figure {
	final double radius;

	Circle(double radius) {
		this.radius = radius;
	}

	public double area() {
		return Math.PI * (radius * radius);
	}
}
