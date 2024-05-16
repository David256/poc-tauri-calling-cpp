<script lang="ts">
  import { invoke } from "@tauri-apps/api";

  const numbers: number[] = [4, 2, 5, 8, 3, -3];
  const strings: string[] = ["xy", "mob", "iron", "rain", "RUN"];

  const showValues = (array: any[]) => array.join(", ");
  const showStrings = (array: any[]) =>
    showValues(array.map((value) => `"${value}"`));

  let resultOfNumbers = 0;
  let resultOfStrings: string[] = [];
  let errorMessage = "";

  async function runTest() {
    try {
      resultOfNumbers = await invoke("sum_array_of_numbers", {
        data: numbers,
      });

      resultOfStrings = await invoke("convert_each_value_to_uppercase", {
        data: strings,
      });

      errorMessage = "";
    } catch (err: any) {
      console.error(err);
      errorMessage = err.toString();
    }
  }
</script>

<main>
  <h1>PoC of Tauri calling C++ functions</h1>
  <p>
    This is a proof of concept. In this test, we try to pass an array of
    integers, and an array of strings to a C++ function. With the numeric array
    the result should be a sum of all the values. For the array of strings, the
    result should be another array strings with the value in format uppercase.
  </p>
  <button on:click={runTest}>Start test</button>

  <div class="box input-box">
    <span class="badge">Inputs:</span>
    <p>
      Numbers: {showValues(numbers)}
    </p>
    <p>
      Strings: {showStrings(strings)}
    </p>
  </div>

  <div class="box output-box">
    <span class="badge">Output:</span>
    <p>
      Numbers: {resultOfNumbers}
    </p>
    <p>
      Strings: {showStrings(resultOfStrings)}
    </p>
  </div>

  <div class="error">
    {errorMessage}
  </div>
</main>

<style>
  .box {
    position: relative;
    border: 1px solid black;
    padding: 1rem;
    margin-top: 1rem;
    margin-bottom: 1rem;
  }
  .badge {
    position: absolute;
    top: 0;
    left: 0;
    background-color: black;
    color: white;
  }
  .error {
    padding: 1rem;
    background-color: black;
    color: white;
    height: 2rem;
    overflow-y: scroll;
  }
</style>
