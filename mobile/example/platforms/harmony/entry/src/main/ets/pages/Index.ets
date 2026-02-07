import junita from 'libexample.so'

@Entry
@Component
struct Index {
  private junitaContext: number = 0

  build() {
    Column() {
      XComponent({
        id: 'junita_canvas',
        type: XComponentType.SURFACE,
        libraryname: 'example'
      })
      .width('100%')
      .height('100%')
      .onLoad((xComponentContext) => {
        console.info('[Junita] XComponent loaded')
        // Initialize Junita native module
        this.junitaContext = junita.init(xComponentContext)
      })
      .onDestroy(() => {
        console.info('[Junita] XComponent destroyed')
        if (this.junitaContext !== 0) {
          junita.destroy(this.junitaContext)
          this.junitaContext = 0
        }
      })
    }
    .width('100%')
    .height('100%')
  }
}
