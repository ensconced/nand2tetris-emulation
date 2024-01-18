# syntax=docker/dockerfile:1.6.0
FROM --platform=linux/amd64 debian:12.4

ENV F4PGA_INSTALL_DIR="/root/opt/f4pga" FPGA_FAM="xc7" F4PGA_TIMESTAMP="20220920-124259" F4PGA_HASH="007d1c1" TARGET="basys3"
# TODO - see which of these we can remove
ENV F4PGA_PACKAGES='install-xc7 xc7a50t_test xc7a100t_test xc7a200t_test xc7z010_test'

RUN apt-get update -y
RUN apt-get install -y git wget xz-utils build-essential
RUN git clone https://github.com/chipsalliance/f4pga-examples
WORKDIR /f4pga-examples
RUN git reset --hard a5a44fa
RUN wget https://repo.continuum.io/miniconda/Miniconda3-latest-Linux-x86_64.sh -O conda_installer.sh
RUN chmod +x conda_installer.sh && ./conda_installer.sh -u -b -p $F4PGA_INSTALL_DIR/$FPGA_FAM/conda
# TODO - remove lots more things to make for a smaller image...
# I should also squash the image when building also...
RUN rm -rf .git conda_installer.sh

# All further RUN commands should use bash, and should source the conda startup stuff first so that
# conda is in the PATH etc.
RUN echo source "$F4PGA_INSTALL_DIR/$FPGA_FAM/conda/etc/profile.d/conda.sh" > /etc/profile
ENV BASH_ENV="/etc/profile"
SHELL ["/bin/bash", "-c"]

RUN conda env create -f $FPGA_FAM/environment.yml
RUN mkdir -p "$F4PGA_INSTALL_DIR/$FPGA_FAM"
RUN for PKG in $F4PGA_PACKAGES; do wget -qO- https://storage.googleapis.com/symbiflow-arch-defs/artifacts/prod/foss-fpga-tools/symbiflow-arch-defs/continuous/install/${F4PGA_TIMESTAMP}/symbiflow-arch-defs-${PKG}-${F4PGA_HASH}.tar.xz | tar -xJC $F4PGA_INSTALL_DIR/${FPGA_FAM}; done
RUN echo conda activate $FPGA_FAM >> /etc/profile

WORKDIR /f4pga-examples/xc7
RUN make -C counter_test
RUN make download -C counter_test
