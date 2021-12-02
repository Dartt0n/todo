# Maintainer: Dartt0n <antonkudryavtsevdoem@gmail.com>
pkgname=dartt0n-td
pkgver=2.3.1
pkgrel=1
pkgdesc="Super fast and simple tasks organizer written in rust"
url="https://github.com/Dartt0n/todo"
license=('GPL')
conflicts=("todo-git"
           "todo-bin")
depends=()
makedepends=()
arch=("x86_64")
source=("https://github.com/Dartt0n/todo/releases/download/${pkgver}/td"
		"https://raw.githubusercontent.com/Dartt0n/todo/master/LICENSE")

package() {
	mkdir -p ${pkgdir}/usr/bin
	mkdir -p ${pkgdir}/usr/share/licenses/${pkgname}

	install -Dm 755 ${srcdir}/td ${pkgdir}/usr/bin/td
	install -Dm 644 ${srcdir}/LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
}

sha256sums=('dbe09cfa796356c56bc8a8a8235a90dd96687f31b5fb2a77378b906736ecb34c'
            '3972dc9744f6499f0f9b2dbf76696f2ae7ad8af9b23dde66d6af86c9dfb36986')
