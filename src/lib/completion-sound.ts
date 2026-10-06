const SOUND_URL = "/audio/task-completed.wav";

let audioContext: AudioContext | undefined;
let decodedSound: AudioBuffer | undefined;
let loadingSound: Promise<void> | undefined;
let pendingPlayback = false;

function playDecodedSound() {
  if (!audioContext || !decodedSound) return;

  const context = audioContext;
  const buffer = decodedSound;
  const startSource = () => {
    try {
      const source = context.createBufferSource();
      const gain = context.createGain();
      gain.gain.value = 0.85;
      source.buffer = buffer;
      source.connect(gain);
      gain.connect(context.destination);
      source.start();
    } catch (error) {
      console.warn("Could not start the task completion sound:", error);
    }
  };

  if (context.state === "running") {
    startSource();
  } else {
    void context
      .resume()
      .then(startSource)
      .catch((error: unknown) => {
        console.warn("Could not resume audio playback:", error);
      });
  }
}

/** Prepare audio decoding before the timer expires so decoding doesn't happen on the completion tick. */
export function prepareCompletionSound(unlockFromUserGesture = false) {
  if (typeof window === "undefined" || typeof AudioContext === "undefined")
    return;

  try {
    audioContext ??= new AudioContext();
    if (unlockFromUserGesture && audioContext.state !== "running") {
      // Resume immediately within the Start button's user-gesture handler.
      void audioContext.resume().catch((error: unknown) => {
        console.warn("Could not unlock task completion audio:", error);
      });
    }
    if (!decodedSound && !loadingSound) {
      loadingSound = fetch(SOUND_URL)
        .then((response) => {
          if (!response.ok)
            throw new Error(`Audio request failed: ${response.status}`);
          return response.arrayBuffer();
        })
        .then((data) => audioContext!.decodeAudioData(data))
        .then((buffer) => {
          decodedSound = buffer;
          if (pendingPlayback) {
            pendingPlayback = false;
            playDecodedSound();
          }
        })
        .catch((error: unknown) => {
          console.warn("Could not prepare the task completion sound:", error);
        })
        .finally(() => {
          loadingSound = undefined;
        });
    }
  } catch (error) {
    console.warn(
      "Web Audio is unavailable for the task completion sound:",
      error,
    );
  }
}

/** Queue playback without awaiting audio device initialization on the timer/UI path. */
export function playCompletionSound() {
  prepareCompletionSound();
  if (decodedSound) {
    playDecodedSound();
  } else {
    pendingPlayback = true;
  }
}
