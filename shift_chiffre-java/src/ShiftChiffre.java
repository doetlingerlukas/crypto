import java.util.Arrays;
import java.util.stream.IntStream;

public class ShiftChiffre {
  private static String alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  public static void main(String[] args) {
    IntStream.rangeClosed(1, 26)
      .forEach(i -> {
        System.out.println("Key "+i+": "+Arrays.stream("UJGUGNNUUGCUJGNNUDAVJGUGCUJQTG".split(""))
          .map(s -> Character.toString(alphabet.charAt(((alphabet.indexOf(s) - i) < 0 ? 26 - Math.abs(alphabet.indexOf(s) - i) : (alphabet.indexOf(s) - i)) % 26)))
          .collect(StringBuilder::new, StringBuilder::append, StringBuilder::append));
      });
  }
}