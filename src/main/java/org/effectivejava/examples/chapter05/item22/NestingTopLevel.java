package org.effectivejava.examples.chapter05.item22;

import java.util.Iterator;
import java.util.NoSuchElementException;

public final class NestingTopLevel implements Iterable<Short> {
	private short a;
	private short b;

	private NestingTopLevel(short a, short b) {
		this.a = a;
		this.b = b;
	}

	static public final class Builder {
		private NestingTopLevel inner;

		public Builder() {
			this.inner = new NestingTopLevel((short)0, (short)0);
		}

		public Builder a(int v) {
			this.inner.a = (short)v;
			return this;
		}

		public Builder b(int v) {
			this.inner.b = (short)v;
			return this;
		}

		public NestingTopLevel build() {
			if (this.inner == null) {
				throw new AssertionError("Cannot return the same thing twice");
			}

			NestingTopLevel tmp = this.inner;
			this.inner = null;
			return tmp;
		}
	}

	public Iterator<Short> iterator() {
		return new Iterator<Short>() {
			private short state;

			public Short next() {
				if (state == 0) {
					state += 1;
					return NestingTopLevel.this.a;
				} else if ( state == 1) {
					state += 1;
					return NestingTopLevel.this.b;
				} else {
					throw new NoSuchElementException();
				}
			}

			public boolean hasNext() {
				return state < 2;
			}

			public void remove() {
				throw new UnsupportedOperationException();
			}
		};
	}

	public static void main(String[] args) {
		NestingTopLevel f = new NestingTopLevel.Builder()
											   .a(4)
											   .b(2)
											   .build();
		for (Short m : f) {
			System.out.println(m);
		}
	}
}