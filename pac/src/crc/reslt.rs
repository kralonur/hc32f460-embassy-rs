#[doc = "Register `RESLT` reader"]
pub type R = crate::R<ResltSpec>;
#[doc = "Register `RESLT` writer"]
pub type W = crate::W<ResltSpec>;
#[doc = "Field `CRC_REG` reader - desc CRC_REG"]
pub type CrcRegR = crate::FieldReader<u16>;
#[doc = "Field `CRC_REG` writer - desc CRC_REG"]
pub type CrcRegW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CRCFLAG_16` reader - desc CRCFLAG_16"]
pub type Crcflag16R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - desc CRC_REG"]
    #[inline(always)]
    pub fn crc_reg(&self) -> CrcRegR {
        CrcRegR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - desc CRCFLAG_16"]
    #[inline(always)]
    pub fn crcflag_16(&self) -> Crcflag16R {
        Crcflag16R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CRC_REG"]
    #[inline(always)]
    pub fn crc_reg(&mut self) -> CrcRegW<'_, ResltSpec> {
        CrcRegW::new(self, 0)
    }
}
#[doc = "desc RESLT\n\nYou can [`read`](crate::Reg::read) this register and get [`reslt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reslt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResltSpec;
impl crate::RegisterSpec for ResltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reslt::R`](R) reader structure"]
impl crate::Readable for ResltSpec {}
#[doc = "`write(|w| ..)` method takes [`reslt::W`](W) writer structure"]
impl crate::Writable for ResltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESLT to value 0"]
impl crate::Resettable for ResltSpec {}
