FROM --platform=linux/amd64 debian:12.4

RUN apt update -y
RUN apt install -y git wget xz-utils
RUN git clone https://github.com/chipsalliance/f4pga-examples
WORKDIR f4pga-examples
RUN git reset --hard a5a44fa
RUN wget https://repo.continuum.io/miniconda/Miniconda3-latest-Linux-x86_64.sh -O conda_installer.sh
ENV F4PGA_INSTALL_DIR="~/opt/f4pga"
ENV FPGA_FAM="xc7"
RUN bash conda_installer.sh -u -b -p $F4PGA_INSTALL_DIR/$FPGA_FAM/conda;
RUN source "$F4PGA_INSTALL_DIR/$FPGA_FAM/conda/etc/profile.d/conda.sh";
RUN conda env create -f $FPGA_FAM/environment.yml







