use leptos::*;

#[tracing::instrument]
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div
        class="bg-ctp-base border-y-2 mt-12"
        style="border-color: hsla(105, 69%, 30%, 50%)"
      >

        <footer >
          <div class="max-w-screen-xl mx-auto py-12 px-4 sm:px-6 lg:py-16 lg:px-8">
            <div class="xl:grid xl:grid-cols-3 xl:gap-8">
              <div class="grid grid-cols-2 gap-8 xl:col-span-2">
                <div class="md:grid md:grid-cols-2 md:gap-8">
                  <div>
                    <h4 class="text-sm leading-5 font-semibold text-gray-400 tracking-wider uppercase">
                      Links
                    </h4>
                    <ul class="mt-4 space-y-4">
                      <li>
                        <a
                          href="https://github.com/ChristopherBiscardi/advent-of-code/tree/main/2023/rust"
                          class="text-base leading-6 text-gray-300 hover:text-white"
                        >
                          GitHub
                        </a>
                      </li>
                      <li>
                        <a
                          href="https://www.youtube.com/playlist?list=PLWtPciJ1UMuD3_8Pb-EqrFhkYpastR2cn"
                          class="text-base text-ctp-red leading-6 hover:text-white"
                        >
                        Solution Videos
                        </a>
                      </li>

                    </ul>
                  </div>
                  <div class="mt-12 md:mt-0">
                    <h4 class="text-sm leading-5 font-semibold text-gray-400 tracking-wider uppercase">
                      Presented By
                    </h4>
                    <ul class="mt-4 space-y-4">
                      <li>
                        <a
                          href="https://rustadventure.dev"
                          class="text-base leading-6 text-ctp-red hover:text-white"
                        >
                          Rust Adventure
                        </a>
                      </li>

                    </ul>
                  </div>
                </div>

              </div>
            </div>
            <div class="mt-8 border-t border-gray-500 pt-8 md:flex md:items-center md:justify-between">
              <div class="flex space-x-6 md:order-2">
                <a
                  href="https://discord.gg/VhffVY5weF"
                  class="text-gray-400 hover:text-gray-300"
                >
                  <span class="sr-only">Hachyderm</span>
                  <svg class="h-6 w-6" fill="currentColor"  viewBox="0 0 245 240"><path d="M104.4 103.9c-5.7 0-10.2 5-10.2 11.1s4.6 11.1 10.2 11.1c5.7 0 10.2-5 10.2-11.1.1-6.1-4.5-11.1-10.2-11.1zm36.5 0c-5.7 0-10.2 5-10.2 11.1s4.6 11.1 10.2 11.1c5.7 0 10.2-5 10.2-11.1s-4.5-11.1-10.2-11.1z" class="st0"/><path d="M189.5 20h-134C44.2 20 35 29.2 35 40.6v135.2c0 11.4 9.2 20.6 20.5 20.6h113.4l-5.3-18.5 12.8 11.9 12.1 11.2 21.5 19V40.6c0-11.4-9.2-20.6-20.5-20.6zm-38.6 130.6s-3.6-4.3-6.6-8.1c13.1-3.7 18.1-11.9 18.1-11.9-4.1 2.7-8 4.6-11.5 5.9-5 2.1-9.8 3.5-14.5 4.3-9.6 1.8-18.4 1.3-25.9-.1-5.7-1.1-10.6-2.7-14.7-4.3-2.3-.9-4.8-2-7.3-3.4-.3-.2-.6-.3-.9-.5-.2-.1-.3-.2-.4-.3-1.8-1-2.8-1.7-2.8-1.7s4.8 8 17.5 11.8c-3 3.8-6.7 8.3-6.7 8.3-22.1-.7-30.5-15.2-30.5-15.2 0-32.2 14.4-58.3 14.4-58.3 14.4-10.8 28.1-10.5 28.1-10.5l1 1.2c-18 5.2-26.3 13.1-26.3 13.1s2.2-1.2 5.9-2.9c10.7-4.7 19.2-6 22.7-6.3.6-.1 1.1-.2 1.7-.2 6.1-.8 13-1 20.2-.2 9.5 1.1 19.7 3.9 30.1 9.6 0 0-7.9-7.5-24.9-12.7l1.4-1.6s13.7-.3 28.1 10.5c0 0 14.4 26.1 14.4 58.3 0 0-8.5 14.5-30.6 15.2z" class="st0"/></svg>
                </a>
                <a
                  href="https://github.com/ChristopherBiscardi/advent-of-code/tree/main/2023/rust"
                  class="text-gray-400 hover:text-gray-300"
                >
                  <span class="sr-only">GitHub</span>
                  <svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">
                    <path
                      fill-rule="evenodd"
                      d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
                      clip-rule="evenodd"
                    />
                  </svg>
                </a>

              </div>
              <p class="mt-8 text-base leading-6 text-gray-400 md:mt-0 md:order-1">
                Built with <a href="https://leptos.dev/">Leptos</a>, <a href="https://github.com/tokio-rs/axum">Axum</a>, and Wasm
              </p>
            </div>
          </div>
        </footer>
      </div>
    }
}
