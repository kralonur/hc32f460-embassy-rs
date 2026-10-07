#[doc = "Register `BLKSIZE` reader"]
pub type R = crate::R<BlksizeSpec>;
#[doc = "Register `BLKSIZE` writer"]
pub type W = crate::W<BlksizeSpec>;
#[doc = "Field `TBS` reader - desc TBS"]
pub type TbsR = crate::FieldReader<u16>;
#[doc = "Field `TBS` writer - desc TBS"]
pub type TbsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - desc TBS"]
    #[inline(always)]
    pub fn tbs(&self) -> TbsR {
        TbsR::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc TBS"]
    #[inline(always)]
    pub fn tbs(&mut self) -> TbsW<'_, BlksizeSpec> {
        TbsW::new(self, 0)
    }
}
#[doc = "desc BLKSIZE\n\nYou can [`read`](crate::Reg::read) this register and get [`blksize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blksize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlksizeSpec;
impl crate::RegisterSpec for BlksizeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`blksize::R`](R) reader structure"]
impl crate::Readable for BlksizeSpec {}
#[doc = "`write(|w| ..)` method takes [`blksize::W`](W) writer structure"]
impl crate::Writable for BlksizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLKSIZE to value 0"]
impl crate::Resettable for BlksizeSpec {}
