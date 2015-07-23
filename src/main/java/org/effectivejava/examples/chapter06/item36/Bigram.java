// Can you spot the bug? - Page 176
package org.effectivejava.examples.chapter06.item36;

import java.util.HashSet;
import java.util.Set;

public class Bigram {
	private final char first;
	private final char second;

	public Bigram(char first, char second) {
		this.first = first;
		this.second = second;
	}

	@Override
	public boolean equals(Object o) {
		if (!(o instanceof Bigram)) return false;
		if (this == o) return true;
		Bigram b = (Bigram) o;
		return b.first == first && b.second == second;
	}

	@Override
	public int hashCode() {
		return 31 * first + second;
	}

	public static void main(String[] args) {
		Set<Bigram> s = new HashSet<Bigram>();
		for (int i = 0; i < 10; i++)
			for (char ch = 'a'; ch <= 'z'; ch++)
				s.add(new Bigram(ch, ch));
		System.out.println(s.size());
	}
}
